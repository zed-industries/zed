#[cfg(feature = "Security_Credentials")]
pub trait IBackgroundTransferBase_Impl: Sized {
    fn SetRequestHeader(&self, headername: &windows_core::HSTRING, headervalue: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn ServerCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetServerCredential(&self, credential: Option<&super::super::Security::Credentials::PasswordCredential>) -> windows_core::Result<()>;
    fn ProxyCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetProxyCredential(&self, credential: Option<&super::super::Security::Credentials::PasswordCredential>) -> windows_core::Result<()>;
    fn Method(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetMethod(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Group(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetGroup(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn CostPolicy(&self) -> windows_core::Result<BackgroundTransferCostPolicy>;
    fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> windows_core::Result<()>;
}
#[cfg(feature = "Security_Credentials")]
impl windows_core::RuntimeName for IBackgroundTransferBase {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferBase";
}
#[cfg(feature = "Security_Credentials")]
impl IBackgroundTransferBase_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundTransferBase_Vtbl
    where
        Identity: IBackgroundTransferBase_Impl,
    {
        unsafe extern "system" fn SetRequestHeader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, headername: core::mem::MaybeUninit<windows_core::HSTRING>, headervalue: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IBackgroundTransferBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundTransferBase_Impl::SetRequestHeader(this, core::mem::transmute(&headername), core::mem::transmute(&headervalue)).into()
        }
        unsafe extern "system" fn ServerCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundTransferBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundTransferBase_Impl::ServerCredential(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, credential: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundTransferBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundTransferBase_Impl::SetServerCredential(this, windows_core::from_raw_borrowed(&credential)).into()
        }
        unsafe extern "system" fn ProxyCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundTransferBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundTransferBase_Impl::ProxyCredential(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, credential: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundTransferBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundTransferBase_Impl::SetProxyCredential(this, windows_core::from_raw_borrowed(&credential)).into()
        }
        unsafe extern "system" fn Method<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IBackgroundTransferBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundTransferBase_Impl::Method(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMethod<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IBackgroundTransferBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundTransferBase_Impl::SetMethod(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Group<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IBackgroundTransferBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundTransferBase_Impl::Group(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IBackgroundTransferBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundTransferBase_Impl::SetGroup(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn CostPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut BackgroundTransferCostPolicy) -> windows_core::HRESULT
        where
            Identity: IBackgroundTransferBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundTransferBase_Impl::CostPolicy(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCostPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: BackgroundTransferCostPolicy) -> windows_core::HRESULT
        where
            Identity: IBackgroundTransferBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundTransferBase_Impl::SetCostPolicy(this, value).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundTransferBase, OFFSET>(),
            SetRequestHeader: SetRequestHeader::<Identity, OFFSET>,
            ServerCredential: ServerCredential::<Identity, OFFSET>,
            SetServerCredential: SetServerCredential::<Identity, OFFSET>,
            ProxyCredential: ProxyCredential::<Identity, OFFSET>,
            SetProxyCredential: SetProxyCredential::<Identity, OFFSET>,
            Method: Method::<Identity, OFFSET>,
            SetMethod: SetMethod::<Identity, OFFSET>,
            Group: Group::<Identity, OFFSET>,
            SetGroup: SetGroup::<Identity, OFFSET>,
            CostPolicy: CostPolicy::<Identity, OFFSET>,
            SetCostPolicy: SetCostPolicy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundTransferBase as windows_core::Interface>::IID
    }
}
pub trait IBackgroundTransferContentPartFactory_Impl: Sized {
    fn CreateWithName(&self, name: &windows_core::HSTRING) -> windows_core::Result<BackgroundTransferContentPart>;
    fn CreateWithNameAndFileName(&self, name: &windows_core::HSTRING, filename: &windows_core::HSTRING) -> windows_core::Result<BackgroundTransferContentPart>;
}
impl windows_core::RuntimeName for IBackgroundTransferContentPartFactory {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferContentPartFactory";
}
impl IBackgroundTransferContentPartFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundTransferContentPartFactory_Vtbl
    where
        Identity: IBackgroundTransferContentPartFactory_Impl,
    {
        unsafe extern "system" fn CreateWithName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundTransferContentPartFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundTransferContentPartFactory_Impl::CreateWithName(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithNameAndFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::HSTRING>, filename: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundTransferContentPartFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundTransferContentPartFactory_Impl::CreateWithNameAndFileName(this, core::mem::transmute(&name), core::mem::transmute(&filename)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundTransferContentPartFactory, OFFSET>(),
            CreateWithName: CreateWithName::<Identity, OFFSET>,
            CreateWithNameAndFileName: CreateWithNameAndFileName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundTransferContentPartFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Storage_Streams")]
pub trait IBackgroundTransferOperation_Impl: Sized {
    fn Guid(&self) -> windows_core::Result<windows_core::GUID>;
    fn RequestedUri(&self) -> windows_core::Result<super::super::Foundation::Uri>;
    fn Method(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Group(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn CostPolicy(&self) -> windows_core::Result<BackgroundTransferCostPolicy>;
    fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> windows_core::Result<()>;
    fn GetResultStreamAt(&self, position: u64) -> windows_core::Result<super::super::Storage::Streams::IInputStream>;
    fn GetResponseInformation(&self) -> windows_core::Result<ResponseInformation>;
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for IBackgroundTransferOperation {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferOperation";
}
#[cfg(feature = "Storage_Streams")]
impl IBackgroundTransferOperation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundTransferOperation_Vtbl
    where
        Identity: IBackgroundTransferOperation_Impl,
    {
        unsafe extern "system" fn Guid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IBackgroundTransferOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundTransferOperation_Impl::Guid(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedUri<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundTransferOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundTransferOperation_Impl::RequestedUri(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Method<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IBackgroundTransferOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundTransferOperation_Impl::Method(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Group<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IBackgroundTransferOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundTransferOperation_Impl::Group(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CostPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut BackgroundTransferCostPolicy) -> windows_core::HRESULT
        where
            Identity: IBackgroundTransferOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundTransferOperation_Impl::CostPolicy(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCostPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: BackgroundTransferCostPolicy) -> windows_core::HRESULT
        where
            Identity: IBackgroundTransferOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundTransferOperation_Impl::SetCostPolicy(this, value).into()
        }
        unsafe extern "system" fn GetResultStreamAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, position: u64, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundTransferOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundTransferOperation_Impl::GetResultStreamAt(this, position) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResponseInformation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundTransferOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundTransferOperation_Impl::GetResponseInformation(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundTransferOperation, OFFSET>(),
            Guid: Guid::<Identity, OFFSET>,
            RequestedUri: RequestedUri::<Identity, OFFSET>,
            Method: Method::<Identity, OFFSET>,
            Group: Group::<Identity, OFFSET>,
            CostPolicy: CostPolicy::<Identity, OFFSET>,
            SetCostPolicy: SetCostPolicy::<Identity, OFFSET>,
            GetResultStreamAt: GetResultStreamAt::<Identity, OFFSET>,
            GetResponseInformation: GetResponseInformation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundTransferOperation as windows_core::Interface>::IID
    }
}
pub trait IBackgroundTransferOperationPriority_Impl: Sized {
    fn Priority(&self) -> windows_core::Result<BackgroundTransferPriority>;
    fn SetPriority(&self, value: BackgroundTransferPriority) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBackgroundTransferOperationPriority {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferOperationPriority";
}
impl IBackgroundTransferOperationPriority_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundTransferOperationPriority_Vtbl
    where
        Identity: IBackgroundTransferOperationPriority_Impl,
    {
        unsafe extern "system" fn Priority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut BackgroundTransferPriority) -> windows_core::HRESULT
        where
            Identity: IBackgroundTransferOperationPriority_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundTransferOperationPriority_Impl::Priority(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: BackgroundTransferPriority) -> windows_core::HRESULT
        where
            Identity: IBackgroundTransferOperationPriority_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundTransferOperationPriority_Impl::SetPriority(this, value).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundTransferOperationPriority, OFFSET>(),
            Priority: Priority::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundTransferOperationPriority as windows_core::Interface>::IID
    }
}

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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>() -> IBackgroundTransferBase_Vtbl {
        unsafe extern "system" fn SetRequestHeader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, headername: core::mem::MaybeUninit<windows_core::HSTRING>, headervalue: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundTransferBase_Impl::SetRequestHeader(this, core::mem::transmute(&headername), core::mem::transmute(&headervalue)).into()
        }
        unsafe extern "system" fn ServerCredential<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTransferBase_Impl::ServerCredential(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerCredential<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, credential: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundTransferBase_Impl::SetServerCredential(this, windows_core::from_raw_borrowed(&credential)).into()
        }
        unsafe extern "system" fn ProxyCredential<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTransferBase_Impl::ProxyCredential(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyCredential<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, credential: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundTransferBase_Impl::SetProxyCredential(this, windows_core::from_raw_borrowed(&credential)).into()
        }
        unsafe extern "system" fn Method<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTransferBase_Impl::Method(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMethod<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundTransferBase_Impl::SetMethod(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Group<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTransferBase_Impl::Group(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundTransferBase_Impl::SetGroup(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn CostPolicy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut BackgroundTransferCostPolicy) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTransferBase_Impl::CostPolicy(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCostPolicy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: BackgroundTransferCostPolicy) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundTransferBase_Impl::SetCostPolicy(this, value).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundTransferBase, OFFSET>(),
            SetRequestHeader: SetRequestHeader::<Identity, Impl, OFFSET>,
            ServerCredential: ServerCredential::<Identity, Impl, OFFSET>,
            SetServerCredential: SetServerCredential::<Identity, Impl, OFFSET>,
            ProxyCredential: ProxyCredential::<Identity, Impl, OFFSET>,
            SetProxyCredential: SetProxyCredential::<Identity, Impl, OFFSET>,
            Method: Method::<Identity, Impl, OFFSET>,
            SetMethod: SetMethod::<Identity, Impl, OFFSET>,
            Group: Group::<Identity, Impl, OFFSET>,
            SetGroup: SetGroup::<Identity, Impl, OFFSET>,
            CostPolicy: CostPolicy::<Identity, Impl, OFFSET>,
            SetCostPolicy: SetCostPolicy::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferContentPartFactory_Impl, const OFFSET: isize>() -> IBackgroundTransferContentPartFactory_Vtbl {
        unsafe extern "system" fn CreateWithName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferContentPartFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTransferContentPartFactory_Impl::CreateWithName(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithNameAndFileName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferContentPartFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::HSTRING>, filename: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTransferContentPartFactory_Impl::CreateWithNameAndFileName(this, core::mem::transmute(&name), core::mem::transmute(&filename)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundTransferContentPartFactory, OFFSET>(),
            CreateWithName: CreateWithName::<Identity, Impl, OFFSET>,
            CreateWithNameAndFileName: CreateWithNameAndFileName::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>() -> IBackgroundTransferOperation_Vtbl {
        unsafe extern "system" fn Guid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTransferOperation_Impl::Guid(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedUri<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTransferOperation_Impl::RequestedUri(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Method<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTransferOperation_Impl::Method(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Group<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTransferOperation_Impl::Group(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CostPolicy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut BackgroundTransferCostPolicy) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTransferOperation_Impl::CostPolicy(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCostPolicy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: BackgroundTransferCostPolicy) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundTransferOperation_Impl::SetCostPolicy(this, value).into()
        }
        unsafe extern "system" fn GetResultStreamAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, position: u64, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTransferOperation_Impl::GetResultStreamAt(this, position) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResponseInformation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTransferOperation_Impl::GetResponseInformation(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundTransferOperation, OFFSET>(),
            Guid: Guid::<Identity, Impl, OFFSET>,
            RequestedUri: RequestedUri::<Identity, Impl, OFFSET>,
            Method: Method::<Identity, Impl, OFFSET>,
            Group: Group::<Identity, Impl, OFFSET>,
            CostPolicy: CostPolicy::<Identity, Impl, OFFSET>,
            SetCostPolicy: SetCostPolicy::<Identity, Impl, OFFSET>,
            GetResultStreamAt: GetResultStreamAt::<Identity, Impl, OFFSET>,
            GetResponseInformation: GetResponseInformation::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferOperationPriority_Impl, const OFFSET: isize>() -> IBackgroundTransferOperationPriority_Vtbl {
        unsafe extern "system" fn Priority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferOperationPriority_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut BackgroundTransferPriority) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTransferOperationPriority_Impl::Priority(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTransferOperationPriority_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: BackgroundTransferPriority) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundTransferOperationPriority_Impl::SetPriority(this, value).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundTransferOperationPriority, OFFSET>(),
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundTransferOperationPriority as windows_core::Interface>::IID
    }
}
